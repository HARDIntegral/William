import { ApplyOptions } from '@sapphire/decorators';
import { Command } from '@sapphire/framework';
import { Message } from 'discord.js';

@ApplyOptions<Command.Options>({
	name: 'ping',
	description: 'Ping pong',
	aliases: ['pong']
})
export class PingCommand extends Command {
	// Register Chat Input and Context Menu command
	public override registerApplicationCommands(registry: Command.Registry) {
		// Register Chat Input command
		registry.registerChatInputCommand({
			name: this.name,
			description: this.description
		});
	}

	// Message command
	public override async messageRun(message: Message) {
		this.sendPing(message);
	}

	private async sendPing(message: Message) {
		const pingMessage = await message.reply({ content: 'Ping?' });

		const content = `Pong! Bot Latency ${Math.round(this.container.client.ws.ping)}ms. API Latency ${
			pingMessage.createdTimestamp - message.createdTimestamp
		}ms.`;
		return pingMessage.edit({ content });	
	}
}