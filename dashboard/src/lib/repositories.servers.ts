import {
  createRepositoryContext,
  repositoryDatabasePath,
} from './persistence/repositories/factory'
import { GraphScopeError } from './project-graph'
import { RepositoryError, requiredText } from './services/repository-error'
import type { GraphScope } from './project-graph-types'

export { RepositoryError } from './services/repository-error'

/** Public API facade; bootstrap composes services and owns connection lifetime. */
export function createRepositoryService(path?: string) {
  const repository = createRepositoryContext(path)
  return {
    registerRepository: (input: unknown) =>
      repository.registerRepository(input),
    listRepositories: () => repository.listRepositories(),
    getArchitecture(
      repositoryId: string,
      scope: GraphScope = { kind: 'repository' },
    ) {
      const id = requiredText(repositoryId, 'repositoryId', 200)
      const graph = repository.getArchitecture(id, scope)
      if (graph === undefined)
        throw new RepositoryError('Repository not found', 404)
      return graph
    },
    close: () => repository.close(),
  }
}

export type RepositoryService = ReturnType<typeof createRepositoryService>
/** Return the path used by the default dashboard repository service. */
export const defaultDatabasePath = () => repositoryDatabasePath()
let service: RepositoryService | undefined
const getService = () => (service ??= createRepositoryService())
/** Register a project in the shared service without indexing its source. */
export const registerRepository = (input: unknown) =>
  getService().registerRepository(input)
/** Read registrations and import times from one database snapshot. */
export const listRepositories = () => getService().listRepositories()
/** Read the scoped graph. Return null if no graph metadata exists, or needsReindex for a legacy graph. Throw 404 for a missing registration. */
export const getArchitecture = (id: string, scope?: GraphScope) =>
  getService().getArchitecture(id, scope)

/** Parse the requested graph scope. Folder and file scopes require a stable key. */
export function architectureScope(parameters: URLSearchParams): GraphScope {
  const kind = parameters.get('kind') ?? 'repository'
  if (kind === 'repository') return { kind }
  if (kind !== 'folder' && kind !== 'file')
    throw new RepositoryError('Invalid graph scope', 400)
  return { kind, key: requiredText(parameters.get('key'), 'key', 4096) }
}

/** Map known repository and scope errors to HTTP responses. Log unexpected errors without exposing their details. */
export const repositoryErrorResponse = (error: unknown) => {
  if (error instanceof GraphScopeError)
    return Response.json({ error: error.message }, { status: 400 })
  if (error instanceof RepositoryError)
    return Response.json({ error: error.message }, { status: error.status })
  console.error(error)
  return Response.json({ error: 'Internal server error' }, { status: 500 })
}
