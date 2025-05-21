import path from "node:path"
import tseslint from "typescript-eslint"
import url from "node:url"
import unusedImports from "eslint-plugin-unused-imports"
import unicornPlugin from "eslint-plugin-unicorn"
import functional from "eslint-plugin-functional"
import suggestMembersPlugin from "@ton-ai-core/eslint-plugin-suggest-members"
import react from "eslint-plugin-react"
import reactHooks from "eslint-plugin-react-hooks"

const __dirname = path.dirname(url.fileURLToPath(import.meta.url))

export default tseslint.config(
    // register plugins
    {
        plugins: {
            ["@typescript-eslint"]: tseslint.plugin,
            ["@unused-imports"]: unusedImports,
            ["suggest-members"]: suggestMembersPlugin,
            functional: functional,
            react: react,
            "react-hooks": reactHooks,
        },
    },

    // add files and folders to be ignored
    {
        ignores: [
            "**/*.js",
            "eslint.config.mjs",
            ".github/*",
            ".yarn/*",
            ".vscode-test/*",
            "dist/*",
            "node_modules/*",
        ],
    },

    tseslint.configs.recommended,
    unicornPlugin.configs["flat/recommended"],

    {
        files: ["**/*.ts", "**/*.tsx"],
        languageOptions: {
            parser: tseslint.parser,
            parserOptions: {
                project: true,
                tsconfigRootDir: __dirname,
                ecmaFeatures: {
                    jsx: true,
                },
                ecmaVersion: "latest",
                sourceType: "module"
            },
        },

        rules: {
            "suggest-members/suggest-members": "error",
            "suggest-members/suggest-imports": "error",
            
            // React rules
            "react/react-in-jsx-scope": "off",
            "react/prop-types": "off",
            
            // TypeScript rules
            "@typescript-eslint/explicit-module-boundary-types": "off",
            "@typescript-eslint/no-empty-function": "off",
            "@typescript-eslint/no-inferrable-types": "off",
            "@typescript-eslint/typedef": [
                "error",
                {parameter: true, memberVariableDeclaration: true},
            ],
            "@typescript-eslint/consistent-generic-constructors": ["error", "type-annotation"],
            "@typescript-eslint/no-unused-vars": [
                "error",
                {
                    argsIgnorePattern: "^_",
                    caughtErrorsIgnorePattern: "^_",
                    varsIgnorePattern: "^_",
                },
            ],
            "@typescript-eslint/explicit-function-return-type": [
                "error",
                {
                    allowExpressions: true,
                },
            ],
            "@typescript-eslint/no-explicit-any": "error",
            "@typescript-eslint/prefer-optional-chain": "off",
            "@typescript-eslint/no-extraneous-class": "off",
            "@typescript-eslint/no-magic-numbers": "off",
            
            // Unused imports
            "@unused-imports/no-unused-imports": "error",
            "no-duplicate-imports": "error",

            // Functional programming rules
            "functional/type-declaration-immutability": [
                "error",
                {
                    rules: [
                        {
                            identifiers: ".+",
                            immutability: "ReadonlyShallow",
                            comparator: "AtLeast",
                        },
                    ],
                },
            ],
            "functional/readonly-type": ["error", "keyword"],

            // Unicorn rules
            "unicorn/no-null": "off",
            "unicorn/prevent-abbreviations": "off",
            "unicorn/no-array-for-each": "off",
            "unicorn/import-style": "off",
            "unicorn/filename-case": "off",
            "unicorn/consistent-function-scoping": "off",
            "unicorn/no-nested-ternary": "off"
        },
    },
) 