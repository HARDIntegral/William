"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.UserListener = void 0;
const framework_1 = require("@sapphire/framework");
const utils_1 = require("../../../lib/utils");
class UserListener extends framework_1.Listener {
    run(payload) {
        (0, utils_1.logSuccessCommand)(payload);
    }
    onLoad() {
        this.enabled = this.container.logger.level <= framework_1.LogLevel.Debug;
        return super.onLoad();
    }
}
exports.UserListener = UserListener;
