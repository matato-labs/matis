/** A repository error with an HTTP status suitable for a client response. */
export class RepositoryError extends Error {
  constructor(
    message: string,
    public readonly status: 400 | 404 | 409,
  ) {
    super(message)
    this.name = 'RepositoryError'
  }
}

/** Trim and validate a required string, using the field name in status 400 errors. */
export const requiredText = (
  value: unknown,
  field: string,
  maxLength: number,
) => {
  if (typeof value !== 'string' || !value.trim())
    throw new RepositoryError(`${field} is required`, 400)
  const text = value.trim()
  if (text.length > maxLength)
    throw new RepositoryError(
      `${field} must be at most ${maxLength} characters`,
      400,
    )
  return text
}
