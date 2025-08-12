# Deployment Setup Guide

## Initial Setup

### 1. Create Branches
```bash
# Create staging and production branches from dev
git checkout dev
git checkout -b staging
git push -u origin staging

git checkout -b production  
git push -u origin production

git checkout dev
```

### 2. DigitalOcean App Platform Setup
Create three apps in DigitalOcean App Platform:

#### Development App
- Use `.do/app-dev.yaml` spec
- Connect to `dev` branch
- Enable auto-deploy on push

#### Staging App  
- Use `.do/app-staging.yaml` spec
- Connect to `staging` branch
- Enable auto-deploy on push

#### Production App
- Use `.do/app-production.yaml` spec  
- Connect to `production` branch
- Enable auto-deploy on push

### 3. Update Repository Settings
- Set `dev` as default branch
- Configure branch protection rules (see `.github/branch-protection.md`)

### 4. Update Repo URLs
Replace `your-username/do-ap-internal-lb-testing` in the `.do/app-*.yaml` files with your actual repository path.

## Workflow

1. **Development**: Work on `dev` branch
2. **Push/PR to dev**: Triggers CI tests
3. **On successful tests**: Auto-merges `dev` → `staging` → `production`
4. **Each environment**: Automatically deploys on push

## Environment Variables

- **Development**: `APP_ENV=development`
- **Staging**: `APP_ENV=staging`  
- **Production**: `APP_ENV=production`

## Instance Configuration

- **Dev**: 2 instances, basic-xxs size
- **Staging**: 2 instances, basic-xs size
- **Production**: 3 instances, basic-s size