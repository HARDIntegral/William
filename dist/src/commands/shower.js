"use strict";
var __decorate = (this && this.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
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
exports.ShowerCommand = void 0;
const decorators_1 = require("@sapphire/decorators");
const framework_1 = require("@sapphire/framework");
const discord_js_1 = require("discord.js");
const __1 = require("..");
let ShowerCommand = class ShowerCommand extends framework_1.Command {
    registerApplicationCommands(registry) {
        registry.registerChatInputCommand({
            name: this.name,
            description: this.description,
            type: discord_js_1.ApplicationCommandType.ChatInput,
        });
    }
    messageRun(message, args) {
        return __awaiter(this, void 0, void 0, function* () {
            this.addRole(message, args);
        });
    }
    addRole(message, args) {
        return __awaiter(this, void 0, void 0, function* () {
            var _a, _b;
            const member = yield args.pick('member');
            const pooRoleID = '1238689960786132994';
            const pooRole = (_a = message.guild) === null || _a === void 0 ? void 0 : _a.roles.resolve(pooRoleID);
            const memberRoleID = '1238904839153651733';
            const memberRole = (_b = message.guild) === null || _b === void 0 ? void 0 : _b.roles.resolve(memberRoleID);
            if (message.author.id === __1.integralID) {
                try {
                    yield member.roles.add(memberRole);
                    yield member.roles.remove(pooRole);
                    return message.reply(`YIPPEE! ${member.user.username} is clean!`);
                }
                catch (error) {
                    return __1.client.logger.error(`${error}`);
                }
            }
            else {
                return message.reply('Nice try!');
            }
        });
    }
};
exports.ShowerCommand = ShowerCommand;
exports.ShowerCommand = ShowerCommand = __decorate([
    (0, decorators_1.ApplyOptions)({
        name: 'shower',
        description: 'take a user out of the poop dungeon',
        aliases: ['clean']
    })
], ShowerCommand);
