import {
  ProjectGraph,
  ProjectGraphProjectNode,
  Tree,
  joinPathFragments,
  workspaceRoot,
} from '@nx/devkit';
import { execSync } from 'node:child_process';
import { relative } from 'node:path';
import { existsSync } from 'node:fs';
import { VersionActions } from 'nx/release';
import { ReleaseGroupWithName } from 'nx/src/command-line/release/config/filter-release-groups';
import { FinalConfigForProject } from 'nx/src/command-line/release/version/release-group-processor';
import { NxReleaseVersionConfiguration } from 'nx/src/config/nx-json';
import { parseCargoToml, stringifyCargoToml } from '../utils/toml';

export default class RustVersionActions extends VersionActions {
  validManifestFilenames = ['Cargo.toml'];

  constructor(
    public releaseGroup: ReleaseGroupWithName,
    public projectGraphNode: ProjectGraphProjectNode,
    public finalConfigForProject: FinalConfigForProject
  ) {
    super(releaseGroup, projectGraphNode, finalConfigForProject);
  }

  async readCurrentVersionFromSourceManifest(tree: Tree): Promise<{
    currentVersion: string;
    manifestPath: string;
  } | null> {
    const packageRoot = this.projectGraphNode.data.root;
    const cargoTomlPath = joinPathFragments(packageRoot, 'Cargo.toml');
    const workspaceRelativeCargoTomlPath = relative(workspaceRoot, cargoTomlPath);

    if (!tree.exists(cargoTomlPath)) {
      throw new Error(
        `The project "${this.projectGraphNode.name}" does not have a Cargo.toml available at ${workspaceRelativeCargoTomlPath}.

To fix this you will either need to add a Cargo.toml file at that location, or configure "release" within your nx.json to exclude "${this.projectGraphNode.name}" from the current release group, or amend the packageRoot configuration to point to where the Cargo.toml should be.`
      );
    }

    const cargoTomlContents = tree.read(cargoTomlPath)!.toString('utf-8');
    const data = parseCargoToml(cargoTomlContents);

    if (!data.package?.version) {
      throw new Error(
        `Unable to determine the current version for project "${this.projectGraphNode.name}" from ${workspaceRelativeCargoTomlPath}, please ensure that the "version" field is set within the [package] section of the Cargo.toml file`
      );
    }

    return {
      currentVersion: data.package.version,
      manifestPath: cargoTomlPath,
    };
  }

  async readCurrentVersionFromRegistry(
    tree: Tree,
    currentVersionResolverMetadata: NxReleaseVersionConfiguration['currentVersionResolverMetadata']
  ): Promise<{
    currentVersion: string | null;
    logText: string;
  } | null> {
    const packageRoot = this.projectGraphNode.data.root;
    const cargoTomlPath = joinPathFragments(packageRoot, 'Cargo.toml');
    const cargoTomlContents = tree.read(cargoTomlPath)!.toString('utf-8');
    const data = parseCargoToml(cargoTomlContents);
    const packageName = data.package?.name;

    if (!packageName) {
      throw new Error(
        `Unable to determine package name for project "${this.projectGraphNode.name}" from Cargo.toml`
      );
    }

    const metadata = currentVersionResolverMetadata;
    const registryUrl =
      typeof metadata?.registry === 'string' ? metadata.registry : 'https://crates.io';

    try {
      // Use cargo search to check if package exists on registry
      const result = execSync(`cargo search "${packageName}" --limit 1 --registry crates-io`, {
        encoding: 'utf-8',
        stdio: 'pipe',
      });

      // Parse cargo search output: "package_name = "version"    # description"
      const match = result.match(/^.*?\s*=\s*"([^"]+)"/);
      if (match && match[1]) {
        return {
          currentVersion: match[1],
          logText: `registry=${registryUrl}`,
        };
      }

      return {
        currentVersion: null,
        logText: `registry=${registryUrl} (package not found)`,
      };
    } catch (error) {
      return {
        currentVersion: null,
        logText: `registry=${registryUrl} (error: ${error})`,
      };
    }
  }

  async readCurrentVersionOfDependency(
    tree: Tree,
    projectGraph: ProjectGraph,
    dependencyProjectName: string
  ): Promise<{
    currentVersion: string | null;
    dependencyCollection: string | null;
  }> {
    const packageRoot = this.projectGraphNode.data.root;
    const cargoTomlPath = joinPathFragments(packageRoot, 'Cargo.toml');
    const cargoTomlContents = tree.read(cargoTomlPath)!.toString('utf-8');
    const data = parseCargoToml(cargoTomlContents);

    // Get the package name of the dependency project
    const dependencyProject = projectGraph.nodes[dependencyProjectName];
    if (!dependencyProject) {
      throw new Error(
        `Unable to find dependency project "${dependencyProjectName}" in project graph`
      );
    }

    const dependencyCargoTomlPath = joinPathFragments(dependencyProject.data.root, 'Cargo.toml');
    const dependencyCargoTomlContents = tree.read(dependencyCargoTomlPath)!.toString('utf-8');
    const dependencyData = parseCargoToml(dependencyCargoTomlContents);
    const dependencyPackageName = dependencyData.package?.name;

    if (!dependencyPackageName) {
      throw new Error(
        `Unable to determine package name for dependency project "${dependencyProjectName}"`
      );
    }

    // Check both dependencies and dev-dependencies
    const dependencies = data.dependencies || {};
    const devDependencies = data['dev-dependencies'] || {};

    let depData = dependencies[dependencyPackageName];
    let dependencyCollection: string | null = 'dependencies';

    if (!depData) {
      depData = devDependencies[dependencyPackageName];
      dependencyCollection = depData ? 'dev-dependencies' : null;
    }

    if (!depData) {
      return {
        currentVersion: null,
        dependencyCollection: null,
      };
    }

    // Handle different dependency formats
    let currentVersion: string | null = null;
    if (typeof depData === 'string') {
      currentVersion = depData;
    } else if (typeof depData === 'object' && (depData as Record<string, unknown>).version) {
      currentVersion = (depData as Record<string, unknown>).version as string;
    }

    return {
      currentVersion,
      dependencyCollection,
    };
  }

  async updateProjectVersion(tree: Tree, newVersion: string): Promise<string[]> {
    const packageRoot = this.projectGraphNode.data.root;
    const cargoTomlPath = joinPathFragments(packageRoot, 'Cargo.toml');
    const workspaceRelativeCargoTomlPath = relative(workspaceRoot, cargoTomlPath);

    const cargoTomlContents = tree.read(cargoTomlPath)!.toString('utf-8');
    const data = parseCargoToml(cargoTomlContents);

    if (!data.package) {
      throw new Error(
        `Unable to update version for project "${this.projectGraphNode.name}": no [package] section found in Cargo.toml`
      );
    }

    data.package.version = newVersion;
    tree.write(cargoTomlPath, stringifyCargoToml(data));

    return [`✍️  New version ${newVersion} written to ${workspaceRelativeCargoTomlPath}`];
  }

  async updateProjectDependencies(
    tree: Tree,
    projectGraph: ProjectGraph,
    dependenciesToUpdate: Record<string, string>
  ): Promise<string[]> {
    const packageRoot = this.projectGraphNode.data.root;
    const cargoTomlPath = joinPathFragments(packageRoot, 'Cargo.toml');
    const cargoTomlContents = tree.read(cargoTomlPath)!.toString('utf-8');
    const data = parseCargoToml(cargoTomlContents);

    let numUpdated = 0;
    const logMessages: string[] = [];

    for (const [dependencyProjectName, newVersion] of Object.entries(dependenciesToUpdate)) {
      // Get the package name of the dependency project
      const dependencyProject = projectGraph.nodes[dependencyProjectName];
      if (!dependencyProject) {
        continue;
      }

      const dependencyCargoTomlPath = joinPathFragments(dependencyProject.data.root, 'Cargo.toml');
      const dependencyCargoTomlContents = tree.read(dependencyCargoTomlPath)!.toString('utf-8');
      const dependencyData = parseCargoToml(dependencyCargoTomlContents);
      const dependencyPackageName = dependencyData.package?.name;

      if (!dependencyPackageName) {
        continue;
      }

      // Update in dependencies section
      if (data.dependencies && data.dependencies[dependencyPackageName]) {
        if (typeof data.dependencies[dependencyPackageName] === 'string') {
          data.dependencies[dependencyPackageName] = newVersion;
          numUpdated++;
        } else if (typeof data.dependencies[dependencyPackageName] === 'object') {
          (data.dependencies[dependencyPackageName] as Record<string, unknown>).version =
            newVersion;
          numUpdated++;
        }
      }

      // Update in dev-dependencies section
      if (data['dev-dependencies'] && data['dev-dependencies'][dependencyPackageName]) {
        if (typeof data['dev-dependencies'][dependencyPackageName] === 'string') {
          data['dev-dependencies'][dependencyPackageName] = newVersion;
          numUpdated++;
        } else if (typeof data['dev-dependencies'][dependencyPackageName] === 'object') {
          (data['dev-dependencies'][dependencyPackageName] as Record<string, unknown>).version =
            newVersion;
          numUpdated++;
        }
      }
    }

    if (numUpdated > 0) {
      tree.write(cargoTomlPath, stringifyCargoToml(data));
      const depText = numUpdated === 1 ? 'dependency' : 'dependencies';
      logMessages.push(
        `✍️  Updated ${numUpdated} ${depText} in ${relative(workspaceRoot, cargoTomlPath)}`
      );
    }

    return logMessages;
  }
}

/**
 * Function called after all projects have been versioned.
 * Updates Cargo.lock file if it exists.
 */
export async function afterAllProjectsVersioned(
  cwd: string,
  opts: {
    dryRun?: boolean;
    verbose?: boolean;
    rootVersionActionsOptions?: Record<string, unknown>;
  }
): Promise<{
  changedFiles: string[];
  deletedFiles: string[];
}> {
  const changedFiles: string[] = [];

  // Update Cargo.lock file if it exists
  const cargoLockPath = joinPathFragments(cwd, 'Cargo.lock');
  if (existsSync(cargoLockPath)) {
    try {
      if (!opts.dryRun) {
        // Run cargo update to refresh the lock file
        execSync('cargo update', {
          cwd,
          stdio: 'pipe',
        });
      }
      changedFiles.push('Cargo.lock');
    } catch (error) {
      if (opts.verbose) {
        console.warn('[nx-rust] Failed to update Cargo.lock:', error);
      }
    }
  }

  return {
    changedFiles,
    deletedFiles: [],
  };
}
