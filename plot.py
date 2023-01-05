from pymongo import MongoClient
import matplotlib.pyplot as plt

if __name__ == '__main__':
    client = MongoClient()
    db = client['weatherlogger']
    collection = db['weatherlog']

    data = list(collection.find({"name": '"Tromsø"'}))

    xs = [d['dt'] for d in data]
    ys = [d['temp'] for d in data]
    ys1 = [d['temp_min'] for d in data]
    ys2 = [d['temp_max'] for d in data]

    plt.plot(xs, ys, label='temp')
    plt.plot(xs, ys1, label='temp_min')
    plt.plot(xs, ys2, label='temp_max')
    plt.legend()
    plt.show()
