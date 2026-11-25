function nested() {
  try {
    try {
      try { work(); } catch {}
    } catch {}
  } catch {
    try { recover(); } catch {}
  } finally {
    try { close(); } catch {}
  }
}
function flat() {
  try { work(); } catch {}
  try {
    try { work(); } catch {}
  } finally { close(); }
  try {
    try { work(); } finally { close(); }
  } catch {}
}
function independent() {
  try {
    const inner = () => { try { work(); } catch {} };
    class Local {
      static { try { work(); } catch {} }
    }
    inner();
  } catch {}
}
