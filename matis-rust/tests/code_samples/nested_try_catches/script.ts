function nested(): void {
  try {
    try { work(); } catch {}
  } finally {
    try { try { close(); } catch {} } catch {}
  }
}
function flat(): void {
  try { work(); } catch {}
  try { close(); } catch {}
}
function independent(): void {
  try {
    const inner = function (): void { try { work(); } catch {} };
    abstract class Local {
      static { try { work(); } catch {} }
    }
    inner();
  } catch {}
}
