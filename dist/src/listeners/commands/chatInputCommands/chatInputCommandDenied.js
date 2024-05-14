"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.UserEvent = void 0;
const framework_1 = require("@sapphire/framework");
class UserEvent extends framework_1.Listener {
    run(_a, _b) {
        return __awaiter(this, arguments, void 0, function* ({ context, message: content }, { interaction }) {
            // `context: { silent: true }` should make UserError silent:
            // Use cases for this are for example permissions error when running the `eval` command.
            if (Reflect.get(Object(context), 'silent'))
                return;
            if (interaction.deferred || interaction.replied) {
                return interaction.editReply({
                    content,
                    allowedMentions: { users: [interaction.user.id], roles: [] }
                });
            }
            return interaction.reply({
                content,
                allowedMentions: { users: [interaction.user.id], roles: [] },
                ephemeral: true
            });
        });
    }
}
exports.UserEvent = UserEvent;
