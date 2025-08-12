# Branch Protection Configuration

To set up branch protection rules in GitHub:

## Staging Branch Protection
1. Go to Settings > Branches
2. Add rule for `staging` branch:
   - Restrict pushes that create files
   - Require status checks to pass: `test` (from CI workflow)
   - Require branches to be up to date before merging
   - Include administrators in restrictions

## Production Branch Protection  
1. Add rule for `production` branch:
   - Restrict pushes that create files
   - Require status checks to pass: `test` (from CI workflow)  
   - Require branches to be up to date before merging
   - Require pull request reviews before merging
   - Include administrators in restrictions

## Required Status Checks
- CI workflow must pass
- All conversations must be resolved
- Branch must be up to date

## Manual Setup Steps
1. Create the `staging` and `production` branches from `dev`
2. Configure branch protection rules as above
3. Update repository settings to set `dev` as default branch
4. Add required secrets for deployments if needed