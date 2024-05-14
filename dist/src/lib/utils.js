"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getSuccessLoggerData = exports.logSuccessCommand = void 0;
const framework_1 = require("@sapphire/framework");
const colorette_1 = require("colorette");
function logSuccessCommand(payload) {
    let successLoggerData;
    if ('interaction' in payload) {
        successLoggerData = getSuccessLoggerData(payload.interaction.guild, payload.interaction.user, payload.command);
    }
    else {
        successLoggerData = getSuccessLoggerData(payload.message.guild, payload.message.author, payload.command);
    }
    framework_1.container.logger.debug(`${successLoggerData.shard} - ${successLoggerData.commandName} ${successLoggerData.author} ${successLoggerData.sentAt}`);
}
exports.logSuccessCommand = logSuccessCommand;
function getSuccessLoggerData(guild, user, command) {
    var _a;
    const shard = getShardInfo((_a = guild === null || guild === void 0 ? void 0 : guild.shardId) !== null && _a !== void 0 ? _a : 0);
    const commandName = getCommandInfo(command);
    const author = getAuthorInfo(user);
    const sentAt = getGuildInfo(guild);
    return { shard, commandName, author, sentAt };
}
exports.getSuccessLoggerData = getSuccessLoggerData;
function getShardInfo(id) {
    return `[${(0, colorette_1.cyan)(id.toString())}]`;
}
function getCommandInfo(command) {
    return (0, colorette_1.cyan)(command.name);
}
function getAuthorInfo(author) {
    return `${author.username}[${(0, colorette_1.cyan)(author.id)}]`;
}
function getGuildInfo(guild) {
    if (guild === null)
        return 'Direct Messages';
    return `${guild.name}[${(0, colorette_1.cyan)(guild.id)}]`;
}
