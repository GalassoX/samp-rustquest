native JSON_Create(source = "");

native JSON_GetInt(jsonid, key[], &dest);
native JSON_SetInt(jsonid, key[], value);

native JSON_GetString(jsonid, key[], dest[], size = sizeof dest);
native JSON_SetString(jsonid, key[], value[]);