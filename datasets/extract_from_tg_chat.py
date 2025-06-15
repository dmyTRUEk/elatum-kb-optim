# extract msg from tg chat export

import json
import sys
from sys import argv as cli_args
from pprint import pprint

def eprint(*args, **kwargs):
	print(*args, file=sys.stderr, **kwargs)

def einput(*args, **kwargs) -> str:
	eprint(*args, end='', **kwargs)
	return input()



filename = cli_args[1]

with open(filename, 'r') as file:
	data = json.load(file)
# eprint(data)

msgs = data['messages']
# eprint(msgs)

# user_ids = {msg['from_id'] if 'from_id' in msg else '' for msg in msgs}
# user_ids.remove('')
# eprint(user_ids)

# users = {user_id: '' for user_id in user_ids}
users = {}
# eprint(users)
for msg in msgs:
	if 'from_id' in msg:
		user_id = msg['from_id']
		user_name = msg['from']
		users[user_id] = user_name
eprint(users)


# TODO: select users?
for i, user in enumerate(users.items()):
	eprint(f'{i}: {user}')

excluded_users_str = einput('Input users to exclude (space sep): ')
excluded_users = [list(users)[int(user_index.strip())] for user_index in excluded_users_str.split()]
eprint(f'{excluded_users = }')


text = []
for msg in msgs:
	t = msg['text']
	if not isinstance(t, str) or t.strip() == '': continue
	if msg['from_id'] in excluded_users: continue
	text.append(t)
	# eprint(t)
# pprint(text)


chars_stats = {}
for t in text:
	for c in t:
		if c in chars_stats:
			chars_stats[c] += 1
		else:
			chars_stats[c] = 1
chars_stats = sorted(chars_stats.items(), key=lambda k_v: k_v[1], reverse=True)
# eprint(chars_stats)


# emoji_stats = []
# for char, count in chars_stats:
# 	if char.isalpha(): continue
# 	if char.isascii(): continue
# 	if not char.isprintable(): continue
# 	if char not in ' 😂😳🧐👁🌚😭🤔👀🗿🤡🥰👺💀😅☺😎🥲😢😬♂😍👍😁😊😋🤨❤🤦😏🤯🤪😄😡🤷😱🥺😐🔥👌🤬😈🤩🥴🥸😔🥳🙂😒🙃❌💔😞×⭕😓😌🙄👊😕😉😃😇🤑😑😵🙏✨✔🤤😶💫🐧☹🤣✅👉🤗🤮🎪🦆😆☠🌝🔫👆👥💎😜◕🇺🇦👈😤😩😖😫🙈💪✓🔝👋🤘🧠🤟🥵🎆🥎🤢🐭😧☃😀😴😰🕯🎂💴⚖🙁😥🤥⛺𝄡🤒🎉👨🦳🥹★💥🦇🦄🤓👎⊕⊖⊗⊘⊙⊜⚽🆕🔪🍊⚠🔸🔷🕑🕒🕓🕔🕕🕖🕗🕘🕙🕚🕛🕜🕞🕟🕠🕡🕢🕤🕥☑👼🏀🌧🙀😝🔔🏆⌛🏕💃😠♀🚀🌎🌏🌍♾🆓😟🥶📎🍓💦🌆😷◆😨😼🐷🎵🚨🐣🨀🎹☄𝄞𝄢⛈🌅⏱🌃🍆👑🆗🙊🪤𝆮🐺😛👹🌸💩🦀🫧🤕😮🪩🫠🌜🫡🫤❓💅☁🪨💯': continue
# 	emoji_stats.append((char, count))
# eprint(emoji_stats)


CHARS_EXTRA  = ' `01234567890-=[]\\;\',./~!@#$%^&*()_+{}|:"<>?–—'
ALPHABET_ENG = 'qwertyuiopasdfghjklzxcvbnm'
ALPHABET_UKR = 'йцукенгшщзхїґфівапролджєячсмитьбю'

ALPHABET = ALPHABET_ENG

selected_msgs = []
for t in text:
	if all(c in ALPHABET + CHARS_EXTRA for c in t) and any(c in ALPHABET for c in t):
		selected_msgs.append(t)
		print(t)
eprint(f'{len(selected_msgs) = }')

