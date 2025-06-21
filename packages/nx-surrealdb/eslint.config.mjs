import js from '@eslint/js';
import tseslint from '@typescript-eslint/eslint-plugin';
import tsparser from '@typescript-eslint/parser';
import jsonParser from 'jsonc-eslint-parser';
import globals from 'globals';

export default [
  {
    ignores: ['dist/**', 'coverage/**', 'node_modules/**'],
  },
  js.configs.recommended,
  {
    files: [
      'src/**/*.ts',
      'src/**/*.tsx',
      'packages/nx-surrealdb/src/**/*.ts',
      'packages/nx-surrealdb/src/**/*.tsx',
    ],
    languageOptions: {
      parser: tsparser,
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
      },
      globals: {
        ...globals.node,
        ...globals.es2021,
      },
    },
    plugins: {
      '@typescript-eslint': tseslint,
    },
    rules: {
      ...tseslint.configs.recommended.rules,
      '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
      '@typescript-eslint/no-explicit-any': 'warn',
    },
  },
  {
    files: [
      'src/**/*.spec.ts',
      'src/**/*.test.ts',
      'packages/nx-surrealdb/src/**/*.spec.ts',
      'packages/nx-surrealdb/src/**/*.test.ts',
    ],
    languageOptions: {
      parser: tsparser,
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
      },
      globals: {
        ...globals.node,
        ...globals.jest,
        ...globals.es2021,
      },
    },
    plugins: {
      '@typescript-eslint': tseslint,
    },
    rules: {
      ...tseslint.configs.recommended.rules,
      '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
      '@typescript-eslint/no-explicit-any': 'warn',
    },
  },
  {
    files: ['src/**/*.json', '*.json'],
    languageOptions: {
      parser: jsonParser,
    },
    rules: {
      // Basic JSON validation only
    },
  },
];
