native JSON_Create();

native JSON_GetInt(jsonid, prop_name[], &dest);
native JSON_SetInt(jsonid, prop_name[], value);

native JSON_GetString(jsonid, key[], dest[], size = sizeof dest);
native JSON_SetString(jsonid, key[], value[]);