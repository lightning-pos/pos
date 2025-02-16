
import type { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
    overwrite: true,
    ignoreNoDocuments: true,
    schema: 'schema.graphql',
    documents: 'app/**/*.gql',
    generates: {
        './lib/graphql/': {
            preset: 'client',
            config: {
                documentMode: 'string'
            }
        },
    }
};

export default config;
