return {
    name             = "main/stable",
    version            = { major = 1, minor = 0, patch = 1 },
        packages = {
        ["jet"] = {
            version = { major = 3, minor = 1, patch = 0 },
            source     = "https://repo.jet.io/${version}/jet.jpk",
        },
        ["diamond-build"] = {
            version     = { major = 1, minor = 0, patch = 1 },
            source     = "https://repo.jet.io/${version}/diamond-build.jpk",
        }
    }
}