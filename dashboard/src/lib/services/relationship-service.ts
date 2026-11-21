import type { RelationshipRepositoryPort } from '../repository-ports'
import type { ArtifactNode } from '../architecture-artifact'
import type { GraphScope } from '../project-graph-types'

/** Read saved edges for graph rendering and dependency analysis. */
export class RelationshipService {
  constructor(private readonly relationships: RelationshipRepositoryPort) {}

  /** Read outgoing edges for the scope and its descendants, or the whole repository. */
  outgoing(id: string, scope: GraphScope, nodes: Array<ArtifactNode>) {
    const keys =
      scope.kind === 'repository'
        ? undefined
        : nodes
            .filter(
              (node) =>
                node.stableKey === scope.key ||
                node.ancestorStableKeys.includes(scope.key),
            )
            .map((node) => node.stableKey)
    return this.relationships
      .outgoing(id, keys)
      .map(({ ordinal: _ordinal, ...edge }) => edge)
  }

  /** Read endpoint pairs for dependency metrics, excluding containment. */
  relations(id: string) {
    return this.relationships.relations(id)
  }

  /** Read runtime import edges for cycle detection. */
  imports(id: string) {
    return this.relationships.imports(id)
  }
}
