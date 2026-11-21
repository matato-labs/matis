import { and, eq, inArray } from 'drizzle-orm'
import { graphFiles } from './schema'
import { selectedRows } from './selected-rows'
import type { AppDatabase } from './connection'
import type { FileRepositoryPort } from '../../repository-ports'

/** Read file metadata from the saved graph. */
export class FileRepository implements FileRepositoryPort {
  constructor(private readonly database: AppDatabase) {}

  /** Return the stored byte counts and hashes for the selected node keys. */
  read(id: string, keys: Array<string>) {
    const read = (batch: Array<string>) =>
      this.database
        .select({
          stableKey: graphFiles.stableKey,
          bytes: graphFiles.bytes,
          sha256: graphFiles.sha256,
        })
        .from(graphFiles)
        .where(
          and(
            eq(graphFiles.repositoryId, id),
            inArray(graphFiles.stableKey, batch),
          ),
        )
        .all()
    return selectedRows(keys, read)
  }
}
