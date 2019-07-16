# Virtual Table and cityIO Observer

Minimal module to repeatedly reproduce cityIO `json` data and report issues with cityIO service over Slack. Useful for testing CityScope without setting a scanner/table.

#Usage

- module will create a `virtual_table` on cityIO endpoint at: `https://cityio.media.mit.edu/api/table/virtual_table`
- the module will create a `json` applicable cityIO latest API
- If module fails to `POST` to cityIO, error messages will be sent periodically to MIT City Science Slack account at `#cityio` channel
- If cityIO is back on, an ok message will be sent

#Running

`$ python3 vt.py`

# Requirements

- python > 3.4
- Requests (`pip3 install requests`)
- slack python API (`pip3 install slackclient==2.0.0` or newer)
