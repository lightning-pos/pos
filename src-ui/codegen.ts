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
                documentMode: 'string',
                scalars: {
                    DbUuid: {
                        input: 'string',
                        output: 'string',
                    },
                    LocalDateTime: {
                        input: 'string',
                        output: 'string',
                    },
                    Money: {
                        input: 'string',
                        output: 'string',
                    },
                    Percentage: {
                        input: 'string',
                        output: 'string',
                    }
                }
            }
        },
    }
};

export default config;
