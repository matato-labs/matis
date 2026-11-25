function nested() {
  try { return <div />; }
  catch { try { return <span />; } catch { return null; } }
}
function flat() {
  try { return <div />; } catch { return null; }
}
function independent() {
  try {
    const inner = () => { try { return <div />; } catch { return null; } };
    return inner();
  } catch { return null; }
}
