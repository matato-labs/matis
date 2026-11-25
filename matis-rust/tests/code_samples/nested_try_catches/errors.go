package samples

func ordinary() error {
    defer func() { recover() }()
    if err := work(); err != nil {
        if other := work(); other != nil {
            return other
        }
        return err
    }
    return nil
}
