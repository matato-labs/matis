import type { GraphRevisionRepositoryPort } from '../repository-ports'

/** Read revision metadata independently of graph content. */
export class GraphRevisionService {
  constructor(private readonly revisions: GraphRevisionRepositoryPort) {}

  /** Look up the latest saved revision for one repository. */
  find(id: string) {
    return this.revisions.find(id)
  }

  /** Read saved revisions keyed by repository ID. */
  read(ids: Array<string>) {
    return new Map(
      this.revisions.read(ids).map((row) => [row.repositoryId, row]),
    )
  }
}
