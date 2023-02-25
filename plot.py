from pymongo import MongoClient
import matplotlib.pyplot as plt
import datetime
import argparse

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--name', type=str, default='"Tromsø"')

    args = parser.parse_args()

    client = MongoClient()
    db = client['weatherlogger']
    collection = db['weatherlog']

    data = list(collection.find({"name": args.name}))

    xs = [datetime.datetime.fromtimestamp(d['dt']) for d in data]
    ys = [d['temp'] for d in data]

    plt.scatter(xs, ys, label='temp')
    plt.legend()
    plt.show()
