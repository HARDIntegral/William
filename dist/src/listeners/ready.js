"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.UserEvent = void 0;
const framework_1 = require("@sapphire/framework");
const colorette_1 = require("colorette");
const dev = process.env.NODE_ENV !== 'production';
class UserEvent extends framework_1.Listener {
    constructor() {
        super(...arguments);
        this.style = dev ? colorette_1.yellow : colorette_1.blue;
    }
    run() {
        this.printBanner();
        this.printStoreDebugInformation();
    }
    printBanner() {
        const success = (0, colorette_1.green)('+');
        const llc = dev ? colorette_1.magentaBright : colorette_1.white;
        const blc = dev ? colorette_1.magenta : colorette_1.blue;
        const line01 = llc('');
        const line02 = llc('');
        const line03 = llc('');
        // Offset Pad
        const pad = ' '.repeat(7);
        console.log(String.raw `
${line01} ${pad}${blc('1.0.0')}
${line02} ${pad}[${success}] Gateway
${line03}${dev ? ` ${pad}${blc('<')}${llc('/')}${blc('>')} ${llc('DEVELOPMENT MODE')}` : ''}
		`.trim());
    }
    printStoreDebugInformation() {
        const { client, logger } = this.container;
        const stores = [...client.stores.values()];
        const last = stores.pop();
        for (const store of stores)
            logger.info(this.styleStore(store, false));
        logger.info(this.styleStore(last, true));
    }
    styleStore(store, last) {
        return (0, colorette_1.gray)(`${last ? '└─' : '├─'} Loaded ${this.style(store.size.toString().padEnd(3, ' '))} ${store.name}.`);
    }
}
exports.UserEvent = UserEvent;
