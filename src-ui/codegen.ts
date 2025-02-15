
import type { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
    overwrite: true,
    ignoreNoDocuments: true,
    schema: 'schema.graphql',
    generates: {
        './lib/graphql/': {
            preset: 'client',
        },
    }
};

export default config;
