fun nested() {
    try {
        try {
            try { work() } catch (e: A) {}
        } catch (e: B) {}
    } catch (e: A) {
        try { work() } catch (c: B) {}
    } catch (e: B) {
        work()
    } finally {
        try { work() } catch (e: A) {}
    }
}
fun flat() {
    try { work() } catch (e: A) {} catch (e: B) {}
    try {
        try { work() } catch (e: A) {}
    } finally { work() }
}
fun independent() {
    try {
        val inner = { try { work() } catch (e: A) {} }
        val local = object : Base() {
            init { try { work() } catch (e: A) {} }
        }
        inner()
    } catch (e: A) {}
}
