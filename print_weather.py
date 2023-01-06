from pymongo import MongoClient
import argparse

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--name', type=str, default='"Tromsø"')

    args = parser.parse_args()

    client = MongoClient()
    db = client['weatherlogger']
    collection = db['weatherlog']

    data = list(collection.find({"name": args.name}))

    latest = max(data, key=lambda x: x['dt'])

    city = latest['name'].strip('"')
    temp = round(latest['temp'], 2)
    humidity = latest['humidity']

    print(f"{city},{temp}°C,{humidity}%")

