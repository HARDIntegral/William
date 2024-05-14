"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
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
exports.client = exports.botRoleID = exports.integralID = void 0;
const framework_1 = require("@sapphire/framework");
const discord_js_1 = require("discord.js");
const env = __importStar(require("../process.env"));
exports.integralID = '746483697066180719';
exports.botRoleID = '1239085205671772284';
exports.client = new framework_1.SapphireClient({
    defaultPrefix: '$',
    intents: [
        discord_js_1.GatewayIntentBits.MessageContent,
        discord_js_1.GatewayIntentBits.Guilds,
        discord_js_1.GatewayIntentBits.GuildMessages,
    ],
    logger: {
        level: framework_1.LogLevel.Debug
    },
    loadMessageCommandListeners: true
});
const main = () => __awaiter(void 0, void 0, void 0, function* () {
    try {
        exports.client.logger.info('Logging in...');
        yield exports.client.login(env.DISCORD_TOKEN);
        exports.client.logger.info('Logged in!');
    }
    catch (error) {
        exports.client.logger.fatal(error);
        yield exports.client.destroy();
        process.exit(1);
    }
});
void main();
