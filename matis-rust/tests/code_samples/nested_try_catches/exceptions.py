def nested():
    try:
        try:
            try:
                work()
            except A:
                pass
        except B:
            pass
    except* A:
        try:
            work()
        except B:
            pass
    except* B:
        pass
    else:
        try:
            work()
        except A:
            pass
    finally:
        try:
            work()
        except A:
            pass

def flat():
    try:
        work()
    except (A, B):
        pass
    except C:
        pass
    try:
        try:
            work()
        except A:
            pass
    finally:
        work()

def independent():
    try:
        def inner():
            try:
                work()
            except A:
                pass
        class Local:
            try:
                work()
            except A:
                pass
        inner()
    except A:
        pass
