import { invoke } from '@tauri-apps/api/core'
import { TypedDocumentString } from './graphql'

export async function gql<TResult, TVariables>(
    query: TypedDocumentString<TResult, TVariables>,
    ...[vars]: TVariables extends Record<string, never> ? [] : [TVariables]
): Promise<TResult> {
    var response: Array<any>

    console.log('yoyo request', query.toString(), vars)

    response = await invoke('graphql', { query, vars })

    console.log('yoyo response', response)

    if (response[1]?.length > 0) {
        throw new Error(response[1][0].message)
    }

    return response[0] as TResult
}
