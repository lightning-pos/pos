import js from '@eslint/js'
import react from 'eslint-plugin-react'
import reactHooks from 'eslint-plugin-react-hooks'
import tseslint from '@typescript-eslint/eslint-plugin'
import tsParser from '@typescript-eslint/parser'
import nextPlugin from '@next/eslint-plugin-next'

// Common style rules to be applied to all files
const styleRules = {
    'indent': ['error', 4],
    'linebreak-style': ['error', 'unix'],
    'quotes': ['error', 'single', { 'avoidEscape': true }],
    'semi': ['error', 'never'],
    'no-trailing-spaces': 'error',
    'eol-last': ['error', 'always']
}

// TypeScript-specific rules
const tsRules = {
    ...styleRules,
    'no-undef': 'off', // TypeScript already checks this
    '@typescript-eslint/no-unused-vars': ['warn']
}

export default [
    js.configs.recommended,
    {
        plugins: {
            react,
            'react-hooks': reactHooks
        },
        settings: {
            react: {
                version: 'detect'
            }
        },
        rules: {
            ...react.configs.recommended.rules,
            'react-hooks/rules-of-hooks': 'error',
            'react-hooks/exhaustive-deps': 'warn',
            'react/react-in-jsx-scope': 'off'
        }
    },
    {
        plugins: {
            '@next/next': nextPlugin
        },
        rules: {
            ...nextPlugin.configs.recommended.rules,
            ...nextPlugin.configs['core-web-vitals'].rules,
            '@next/next/no-img-element': 'error',
            '@next/next/no-html-link-for-pages': 'error'
        }
    },
    {
        ignores: [
            'node_modules/**',
            '.next/**',
            'out/**',
            'public/**',
        ],
    },
    {
        files: ['**/*.js', '**/*.jsx'],
        languageOptions: {
            ecmaVersion: 'latest',
            sourceType: 'module',
            parserOptions: {
                ecmaFeatures: {
                    jsx: true,
                },
            },
        },
        rules: {
            ...styleRules,
            // Add any specific rules here
        },
    },
    {
        files: ['**/*.ts', '**/*.tsx'],
        plugins: {
            '@typescript-eslint': tseslint
        },
        languageOptions: {
            parser: tsParser,
            parserOptions: {
                ecmaVersion: 'latest',
                sourceType: 'module',
                ecmaFeatures: {
                    jsx: true,
                },
                project: './tsconfig.json',
            },
        },
        rules: {
            ...tseslint.configs.recommended.rules,
            ...tsRules,
            // Add any specific rules here
        },
    },
    {
        files: ['postcss.config.js'],
        languageOptions: {
            globals: {
                module: 'writable',
            },
        },
    },
]
