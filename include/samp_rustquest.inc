//-> JSONs
native JSON_Create(source[] = "");

native JSON_GetInt(jsonid, key[], &dest);
native JSON_SetInt(jsonid, key[], value);

native JSON_GetString(jsonid, key[], dest[], size = sizeof dest);
native JSON_SetString(jsonid, key[], value[]);

native JSON_CreateArray(source[] = "");
native JSON_ArrayStringify(arrid, dest[], size = sizeof dest);
native JSON_ArrayAddInt(arrid, value);
native JSON_ArrayAddFloat(arrid, Float:value);
native JSON_ArrayAddString(arrid, value[]);
native JSON_ArrayIsNumber(arrid, index);
native JSON_ArrayIsString(arrid, index);
native JSON_ArrayGetInt(arrid, index, &dest);
native JSON_ArrayGetFloat(arrid, index, &Float:dest);
native JSON_ArrayGetString(arrid, index, dest[], size = sizeof dest);

//-> Request
#define HTTP_GET            "GET"
#define HTTP_POST           "POST"
#define HTTP_PUT            "PUT"
#define HTTP_PATCH          "PATCH"
#define HTTP_DELETE         "DELETE"
native Request(url[], callback[], method[], data[] = "");