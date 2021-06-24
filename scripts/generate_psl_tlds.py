import requests
import pickle


response = requests.get(
    url='https://publicsuffix.org/list/public_suffix_list.dat',
)
psl_file_data = response.text

psl_tlds = set()
for line in psl_file_data.splitlines():
    stripped_line = line.strip()
    if stripped_line and not stripped_line.startswith(
        (
            '!',
            '*',
            '//',
        )
    ):
        psl_tlds.add(stripped_line)

with open('psl_tlds.pkl', 'wb') as psl_tlds_file:
    pickle.dump(
        obj=psl_tlds,
        file=psl_tlds_file,
    )
