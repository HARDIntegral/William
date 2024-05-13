import { ApplyOptions } from '@sapphire/decorators';
import { Command,  Args } from '@sapphire/framework';
import { 
    ApplicationCommandType, 
    GuildMember, 
    Role, 
    Message 
} from 'discord.js';

@ApplyOptions<Command.Options>({
    name: 'stinkify',
    description: 'Send a user to the poop dungeon',
    aliases: ['poo']
})
export class StinkifyCommand extends Command {
    public override registerApplicationCommands(registry: Command.Registry) {
        registry.registerChatInputCommand({
            name: this.name,
            description: this.description,
            type: ApplicationCommandType.ChatInput,
        });
    }

    public override async messageRun(message: Message, args: Args) {
        this.addRole(message, args);
    }

    private async addRole(message: Message, args: Args) {
        const member: GuildMember = await args.pick('member');
        const pooRoleID: string = '1238689960786132994';
        const pooRole: Role = message.guild?.roles.resolve(pooRoleID as `${bigint}`) as Role;
        const memberRoleID: string = '1238904839153651733';
        const memberRole: Role = message.guild?.roles.resolve(memberRoleID as `${bigint}`) as Role;

        try {
            await member.roles.add(pooRole);
            await member.roles.remove(memberRole);
            return message.reply(`EWWWW! ${member.user.username} is stinky!`);
        } catch (error) {
            return message.reply(`Error: ${error}`);
        }
    }
}
