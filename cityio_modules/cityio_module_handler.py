"""

Modules handler for CityScope:
This is a boilerplate tool for running a CityScope module.
It is meant to shell the interaction with cityIO API
and let module makers focus only on the module itself.

@author: Ariel Noyman @relno
"""
import socket
import json
import os
import time
import subprocess
import requests
from requests.exceptions import HTTPError
import slack
from slack import WebClient


class CityioModulesHandler:
    def __init__(self, settings):
        self.hash_checker = None
        self.settings = settings
        self.CITYIO_GET_MODULE_HASH_URL = self.parse_json_file(
            'base_url') + self.parse_json_file('get_suffix') +\
            self.parse_json_file('table')+'/meta'
        self.CITYIO_POST_MODULE_DATA_URL = self.parse_json_file(
            'base_url') + self.parse_json_file('post_suffix') +\
            self.parse_json_file('table') + self.parse_json_file('module')
        self.CITYIO_MODULE_URL = self.parse_json_file(
            'base_url') + self.parse_json_file('get_suffix') +\
            self.parse_json_file('table') + self.parse_json_file('module')
        self.INTERVAL = self.parse_json_file('interval')

    def get_ip_address(self):
        """gets user's IP"""
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        s.connect(("8.8.8.8", 80))
        return str(s.getsockname()[0])

    def cityio_get_request(self, url):
        """handle get reuests to cityIO"""
        try:
            response = requests.get(url)
            # If the response was successful,
            # no Exception will be raised
            response.raise_for_status()
        except HTTPError as http_err:
            print(f'HTTP error occurred: {http_err}')
        except Exception as err:
            print(f'Other error occurred: {err}')
        else:
            return response.json()

    def cityio_post_request(self, url, data):
        """handle post reuests to cityIO"""
        try:
            response = requests.post(url, data)
            print('cityio POST response:', response)
            response.raise_for_status()
        except HTTPError as http_err:
            print(f'HTTP error occurred: {http_err}')
        except Exception as err:
            print(f'Other error occurred: {err}')

    def get_folder_path(self):
        """get this app folder"""
        loc = str(os.path.realpath(
            os.path.join(os.getcwd(), os.path.dirname(__file__)))) + '/'
        return loc

    def parse_json_file(self, field):
        RES = None
        c = self.get_folder_path()+self.settings
        with open(c) as d:
            data = json.load(d)
            RES = data[field]
        return RES

    def human_time(self, this_time):
        """converts to human time"""
        human_time = time.strftime(
            '%Y/%m/%d_%H:%M:%S', time.localtime(this_time))
        return human_time

    def send_slack_msg(self, msg):
        """send massage on slack
            https://api.slack.com/apps/AJPN6J83W/install-on-team?success=1
            - cd ~
            - nano .bash_profile
            - connect to the api and create client
            - replace slack token with the one from https://api.slack.com/apps/ALJ2FH9RV/install-on-team?
        """
        SLACK_TOKEN = self.parse_json_file('slack_token')
        client = slack.WebClient(SLACK_TOKEN)
        # test the slack api
        client.api_call("auth.test")
        # send to slack
        response = client.chat_postMessage(channel='#cityio', text=msg)
        assert response["ok"]

    def get_hash(self):
        """
        gets the hashes from cityIO
        and check if we got new one
        """

        print('\n')
        META = self.cityio_get_request(self.CITYIO_GET_MODULE_HASH_URL)
        HASH = META['hashes'][self.parse_json_file(
            'hash_to_listen')]
        if self.hash_checker != HASH:
            print('new hash', self.hash_checker)
            self.run_module()
            self.hash_checker = HASH
            return self.hash_checker
        else:
            print('same hash')

    def run_module(self):
        """
        runs the command line module program.
        """

        command = self.parse_json_file('module_command')
        print('running command: "' + command + '"')

        # run the module in a different shell
        process = subprocess.Popen(
            command.split(), stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        result, error = process.communicate()

        if process.returncode != 0:
            err_msg = 'error when executing module:', str(error),
            "at:", self.human_time(time.time())
            print(err_msg)
            self.send_slack_msg(err_msg)
        else:
            results_json = self.parse_json_file('results_json')
            data = open(results_json, 'rb').read()
            self.cityio_post_request(self.CITYIO_POST_MODULE_DATA_URL, data)

    def main(self):
        """main loop"""

        # start the app
        init_msg = 'Starting CityScope module handler for "' + self.parse_json_file('module') +\
            '" running at: ' + \
            self.get_ip_address() + " sending results to: " + \
            self.CITYIO_MODULE_URL
        self.send_slack_msg(init_msg)
        print(init_msg)
        while True:
            self.get_hash()
            time.sleep(self.INTERVAL)


if __name__ == '__main__':
    # clear screen
    os.system('cls' if os.name == 'nt' else "printf '\033c'")
    settings = 'settings.json'
    # load settings file
    cmh = CityioModulesHandler(settings)
    # run the app
    cmh.main()
