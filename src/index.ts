import { Command } from "commander";


import New from "@lib/new";

let cli = new Command("jet")
    .version("v1.0.0-rc")
    .description("the platinummind command line utility")


cli.command("new")
    .description("create new project")
    .action(async function () {
        await New();
    })  


