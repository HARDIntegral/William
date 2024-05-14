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
exports.PingCommand = void 0;
const decorators_1 = require("@sapphire/decorators");
const framework_1 = require("@sapphire/framework");
let PingCommand = class PingCommand extends framework_1.Command {
    // Register Chat Input and Context Menu command
    registerApplicationCommands(registry) {
        // Register Chat Input command
        registry.registerChatInputCommand({
            name: this.name,
            description: this.description
        });
    }
    // Message command
    messageRun(message) {
        return __awaiter(this, void 0, void 0, function* () {
            this.sendPing(message);
        });
    }
    sendPing(message) {
        return __awaiter(this, void 0, void 0, function* () {
            const pingMessage = yield message.reply({ content: 'Ping?' });
            const content = `Pong! Bot Latency ${Math.round(this.container.client.ws.ping)}ms. API Latency ${pingMessage.createdTimestamp - message.createdTimestamp}ms.`;
            return pingMessage.edit({ content });
        });
    }
};
exports.PingCommand = PingCommand;
exports.PingCommand = PingCommand = __decorate([
    (0, decorators_1.ApplyOptions)({
        name: 'ping',
        description: 'Ping pong',
        aliases: ['pong']
    })
], PingCommand);
