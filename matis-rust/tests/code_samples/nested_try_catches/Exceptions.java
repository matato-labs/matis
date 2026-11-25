class Exceptions {
    void nested() {
        try (var resource = open()) {
            try { try { work(); } catch (A | B e) {} } catch (C e) {}
        } catch (A e) {
            try (var resource = open()) { work(); } catch (B ignored) {}
        } catch (B e) {
        } finally {
            try { work(); } catch (C e) {}
        }
    }
    void flat() {
        try { work(); } catch (A | B e) {} catch (C e) {}
        try (var resource = open()) { try { work(); } catch (A e) {} }
        try { try (var resource = open()) { work(); } } catch (A e) {}
    }
    void independent() {
        try {
            Runnable inner = () -> { try { work(); } catch (A e) {} };
            Object local = new Object() {{ try { work(); } catch (A e) {} }};
            record Local(int value) {
                Local { try { work(); } catch (A e) {} }
            }
            inner.run();
        } catch (A e) {}
    }
}
