import { LogLevel, SapphireClient } from '@sapphire/framework';
import { GatewayIntentBits } from 'discord.js';

import * as env from '../process.env';

const client = new SapphireClient({
    defaultPrefix: '$',
    intents: [
        GatewayIntentBits.MessageContent,
        GatewayIntentBits.Guilds, 
        GatewayIntentBits.GuildMessages,
    ],
    logger: {
        level: LogLevel.Debug
    },
    loadMessageCommandListeners: true
});

const main = async () => {
    try {
        client.logger.info('Logging in...');
        await client.login(env.DISCORD_TOKEN);
        client.logger.info('Logged in!');
    } catch (error) {
        client.logger.fatal(error);
        await client.destroy();
        process.exit(1);
    }
};

void main();