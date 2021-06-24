import requests
import pickle


response = requests.get(
    url='http://www.internic.net/domain/root.zone',
)
root_zone_file_data = response.text

ns_records = {}
a_records = {}
for line in root_zone_file_data.splitlines():
    if '\tIN\tNS\t' in line:
        splitted_line = line.split('\t')
        host_label = splitted_line[0].rstrip('.')
        record_data = splitted_line[-1].rstrip('.')
        if host_label in ns_records:
            ns_records[host_label].append(record_data)
        else:
            ns_records[host_label] = [record_data]
    elif '\tIN\tA\t' in line:
        splitted_line = line.split('\t')
        host_label = splitted_line[0].rstrip('.')
        record_data = splitted_line[-1]
        if host_label in a_records:
            a_records[host_label].append(record_data)
        else:
            a_records[host_label] = [record_data]

tld_to_nameservers_ips = {}
for tld, nameservers in ns_records.items():
    tld = tld.strip()
    if tld:
        tld_to_nameservers_ips[tld] = []
        for nameserver in nameservers:
            tld_to_nameservers_ips[tld] += a_records.get(nameserver, [])

with open('root_tld_to_nameservers_ips.pkl', 'wb') as root_tld_to_nameservers_ips_file:
    pickle.dump(
        obj=tld_to_nameservers_ips,
        file=root_tld_to_nameservers_ips_file,
    )
