# Apps

This directory contains deployable applications in the goodie-bag monorepo.

## Structure

- **Websites**: goodiebag.dev, documentation sites, landing pages
- **Web Applications**: membership portals, admin dashboards, tools
- **APIs**: Backend services, serverless functions
- **Mobile Apps**: Future mobile applications

## Deployment

Each app has its own deployment strategy:

- **Static Sites**: Vercel, Netlify, S3 + CloudFront
- **Web Apps**: Vercel, Railway, AWS
- **APIs**: Serverless functions, containers

## Commands

```bash
# Serve app locally
nx serve app-name

# Build for production
nx build app-name

# Deploy app
nx deploy app-name

# Test app
nx test app-name
```

Apps can depend on packages and libs within the monorepo for shared
functionality.
