using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.IO;
using MCGalaxy.Config;
using MCGalaxy.Commands;
using System.Linq;

namespace MCGalaxy {
    public sealed partial class CommandGuiHelperPlugin : Plugin {
        public override string name => "CommandGuiHelper";
        public override string MCGalaxy_Version => "1.9.2.2";
        public override string creator => "SpiralP";

        private CommandGuiHelperCommand command = null;
        public override void Load(bool startup) {
            command = new CommandGuiHelperCommand();
            Command.Register(command);
        }

        public override void Unload(bool shutdown) {
            if (command != null) {
                _ = Command.Unregister(command);
                command = null;
            }
        }

        private class CommandGuiHelperCommand : Command2 {
            public override string name => "GenerateCommandGuiCommands";
            public override string type => CommandTypes.Other;
            public override bool MessageBlockRestricted => true;
            public override LevelPermission defaultRank => LevelPermission.Operator;

            public override void Help(Player p) {
                p.Message("%T/GenerateCommandGuiCommands");
                p.Message("%H  Create json file for use in client CommandGui plugin.");
            }

            struct CommandInfo {
                public string name;
                public string type;
                public string shortcut;
                public LevelPermission defaultRank;
                public string[] help;
                public Alias[] aliases;
                public Perm[] extraPerms;

                public struct Alias {
                    public string trigger, format;
                }

                public struct Perm {
                    public LevelPermission perm;
                    public string description;
                }
            }

            public override void Use(Player p, string message, CommandData data) {
                Dictionary<string, CommandInfo> dict = new Dictionary<string, CommandInfo>();

                foreach (var command in Command.CopyAll()) {
                    if (command.name == this.name) continue;

                    var bufferPlayer = new MessageBufferPlayer();

                    command.Help(bufferPlayer);

                    var aliases = command.Aliases != null
                        ? command.Aliases.Select(alias =>
                            new CommandInfo.Alias {
                                trigger = alias.Trigger,
                                format = alias.Format
                            }).ToArray()
                        : new CommandInfo.Alias[] { };

                    var extraPerms = command.ExtraPerms != null
                        ? command.ExtraPerms.Select(perm =>
                            new CommandInfo.Perm {
                                perm = perm.Perm,
                                description = perm.Description
                            }).ToArray()
                        : new CommandInfo.Perm[] { };

                    CommandInfo info = new CommandInfo {
                        name = command.name,
                        type = command.type,
                        shortcut = command.shortcut == "" ? null : command.shortcut,
                        defaultRank = command.defaultRank,
                        help = bufferPlayer.buffer.ToArray(),
                        aliases = aliases,
                        extraPerms = extraPerms,
                    };

                    dict.Add(command.name, info);
                }

                string json = JsonConvert.SerializeObject(dict, Formatting.Indented);
                string filename = "commands.json";
                File.WriteAllText(filename, json);
                p.Message("wrote to file {0}", filename);
            }

            private sealed class MessageBufferPlayer : Player {
                public readonly List<string> buffer = new List<string>();

                public MessageBufferPlayer() : base("Player") {
                    group = Group.GuestRank;
                    color = "&S";
                    SuperName = "Player";
                }

                public override void Message(byte type, string message) {
                    // Message should start with server color if no initial color
                    if (message.Length > 0 && !(message[0] == '&' || message[0] == '%')) {
                        message = Server.Config.DefaultColor + message;
                    }
                    message = Chat.Format(message, this);

                    try {
                        message = LineWrapper.CleanupColors(message, this);

                        buffer.Add(message);
                    } catch (Exception e) {
                        Logger.LogError(e);
                    }
                }
            }
        }

    } // class CommandGuiHelperPlugin

} // namespace MCGalaxy

