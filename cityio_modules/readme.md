# Modules handler for CityScope

This is a boilerplate tool for running a CityScope module. It is meant to shell the interaction with cityIO API and let module makers focus only on the module itself.

## What is it for?

This tool will listen to changes in a CityScope table (through an end-point on cityIO). Upon a change, the tool will run a module program on the command line. When the program is done computing, the tool will send its results to a designated cityIO end-point.\
The tool is mostly meant to be used for binary/non-dynamic modules that might have hard time interacting with cityIO API easily. It should also be used as a starting point for development of new tools.

## Usage

- install packages
  - To produce a list of needed packages, use `pipreqs`, follow instructions https://github.com/bndr/pipreqs
  - Or simply run the app and install packages as they appear in errors.
- make sure to write your module results into file on path
- run `$ python3 cityio_modules.py`

## Settings

Settings are in `settings.json` (see example in folder):

|            **"table"**             |      **"module"**       |                           **"hash_to_listen"**                            |                   **"interval"**                   |                           **"slack" & "slack_token"**                            |       **"base_url"**        |                      **"post_suffix"**                       |              **"get_suffix"**              |               **"module_command"**               |                **"results_json"**                |                                                                                                 **"hidden_table" & "hidden_table_header"**                                                                                                 |
| :--------------------------------: | :---------------------: | :-----------------------------------------------------------------------: | :------------------------------------------------: | :------------------------------------------------------------------------------: | :-------------------------: | :----------------------------------------------------------: | :----------------------------------------: | :----------------------------------------------: | :----------------------------------------------: | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: |
| cityIO table end-point :: `string` | module name :: `string` | which hash on that table affects this module (usually `grid`) :: `string` | interval for checking changes on cityIO :: `float` | slack token for notifications on cityIO channel (DON'T COMMIT THAT!) :: `string` | cityIO base URL :: `string` | cityIO POST suffix, probably `/api/table/update` :: `string` | cityIO GET suffix `/api/table` :: `string` | command that runs module in terminal :: `string` | file name to look for module results :: `string` | a cityIO token for hidden CityScope tables. Get this token from a cityIO admin if you're to develop a CityScope module that need access to a hidden table. format: "Authorization": "Bearer CITYIO-TOKEN-HERE-NEVER-COMMIT-IT" :: `object` |

# Expected Output

If all goes well, this should appear in terminal:

```

new hash a0f4baaad1054f74678a32aaa0cd5f7dc3da12e3435b91e6d6bc0c7becdcc40b
running command: "sh module.sh"
cityio POST response: <Response [200]>


new hash 4f0894645a87f897e384c353ac38bfa54a66667b7ce6834d885a020070f9821c
running command: "sh module.sh"
cityio POST response: <Response [200]>


new hash b248adc3d3bda26da5b1fd74e436e3ee6345c38e3c10ffb59f2fc73e0125df75
running command: "sh module.sh"
cityio POST response: <Response [200]>

...
```

# Errors

- cityIO channel on slack should report for some events, such as starting this app or failing to run the command line module.
