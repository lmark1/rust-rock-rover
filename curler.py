#!/usr/bin/env python

import requests
import time

zagreb_url ='https://hr.rechargespots.eu/DuskyWebApi//noAuthGeoHashLocations?IsFavorite=false&UserGPSaccessLatitude=45.8150108&UserGPSaccessLongitude=15.981919&connectorTypes=6,5,1,3,17,7,12,13,15,14,9,16,11,10,8,0&northEastLatitude=46.06269702309713&northEastLongitude=16.259278245605476&poiTypes=&searchLocation=Zagreb,+Croatia&searchRadius=2000&showAlsoRoaming=true&southWestLatitude=45.62214864951481&southWestLongitude=15.66944975439454'


def make_request(url):
    headers = {
      'accept': 'application/json, text/plain, */*', 
      'accept-language': 'en-GB,en;q=0.9,hr;q=0.8,en-US;q=0.7,bs;q=0.6,pt;q=0.5', 
      'authorization': 'null', 
      'referer': 'https://hr.rechargespots.eu/', 
      'sec-ch-ua': '"Google Chrome";v="123", "Not:A-Brand";v="8", "Chromium";v="123"', 
      'sec-ch-ua-mobile': '?0', 
      'sec-ch-ua-platform': '"Linux"', 
      'sec-fetch-dest': 'empty', 
      'sec-fetch-mode': 'cors', 
      'sec-fetch-site': 'same-origin', 
      'user-agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36'
    }
    response = requests.get(url, headers=headers)
    time.sleep(0.5)
    return response.json()


data = make_request(zagreb_url)
for el in data:
    lat = el['Latitude']
    lon = el['Longitude']
    template_url = 'https://hr.rechargespots.eu/DuskyWebApi//noAuthGeoHashLocations?UserGPSaccessLatitude={}&UserGPSaccessLongitude={}&northEastLatitude={}&northEastLongitude={}&poiTypes=&searchRadius=2000&showAlsoRoaming=true&southWestLatitude={}&southWestLongitude={}'.format(lat, lon, lat, lon, lat, lon)
    print(make_request(template_url))
