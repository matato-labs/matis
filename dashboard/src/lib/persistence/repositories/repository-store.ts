import { eq } from 'drizzle-orm'
import { repositories } from './schema'
import type { AppDatabase } from './connection'
import type { RepositoryRepositoryPort } from '../../repository-ports'
import type { RepositoryRecord } from '../../graph-records'

/** Persist registrations separately from graph metadata. */
export class RepositoryStore implements RepositoryRepositoryPort {
  constructor(private readonly database: AppDatabase) {}

  /** Insert a registration. Return undefined on a conflict without changing the existing row. */
  register(input: RepositoryRecord) {
    return this.database
      .insert(repositories)
      .values(input)
      .onConflictDoNothing()
      .returning()
      .all()
      .at(0)
  }

  /** Look up the registration by exact ID. */
  find(id: string) {
    return this.database
      .select()
      .from(repositories)
      .where(eq(repositories.id, id))
      .get()
  }

  /** Return registrations ordered by creation time, then ID. */
  list() {
    return this.database
      .select()
      .from(repositories)
      .orderBy(repositories.createdAt, repositories.id)
      .all()
  }
}
