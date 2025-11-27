use indexmap::IndexMap;

pub type MultipleValuesMap = IndexMap<String, Vec<String>>;

const MESSAGE_KEY: &'static str = "message";

/// Parseia o conteúdo de um objeto commit em um mapa.
/// 
/// Neste mapa, a mesma chave pode ter multiplos valores. Portanto o mapa vai de
/// String -> Vec<String>
/// 
/// A ordem de inserção deve ser preservada para que o hash do objeto não mude.
/// 
/// ## Formato do objeto
/// - Cada linha contém um par de chave e valor. A primeira palavra é a chave.
/// - Se um valor contém multiplas linhas, cada linha subsequente deve começar com espaço
/// (o espaço não é incluído no valor)
/// - O mapeamento termina com uma linha vazia. O restante é considerado a mensagem do commit
pub fn key_value_parse(content: &String) -> MultipleValuesMap {
    let mut result = MultipleValuesMap::new();
    key_value_parse_helper(content, &mut result);
    result
}

/// O inverso do parse. Converte o mapa na string do arquivo do objeto
pub fn key_value_serialize(map: &MultipleValuesMap) -> String {
    let mut result = String::new();

    for (key, values) in map {
        if key == MESSAGE_KEY {
            continue;
        }

        for value in values {
            result.push_str(&key);
            result.push(' ');

            let mut value_remainder = value.as_str();

            while let Some(new_line) = value_remainder.find('\n') {
                result.push_str(&value_remainder[..=new_line]);
                result.push(' ');
                value_remainder = &value_remainder[new_line+1..];
            }

            result.push_str(&value_remainder);
            result.push('\n');
        }
    }

    if let Some(message) = map.get(MESSAGE_KEY) {
        if let Some(message) = message.first() {
            result.push('\n');
            result.push_str(message);
        }
    }

    result
}

fn key_value_parse_helper(content: &str, result: &mut MultipleValuesMap) {
    let space = content.find(' ');
    let new_line = content.find('\n');

    if space.is_none() || new_line.is_none() {
        insert_or_append(MESSAGE_KEY, content, result);
        return;
    }

    let space = space.unwrap();
    let new_line = new_line.unwrap();

    let line = &content[..new_line];
    let first_byte = line.bytes().nth(0);

    if first_byte.is_none() {
        insert_or_append(MESSAGE_KEY, content, result);
        return;
    }

    let first_byte = first_byte.unwrap();

    if first_byte == b' ' {
        let line = &content[1..new_line];
        let (_, value) = result.last_mut().expect("Arquivo de objeto deveria estar bem formatado");
        value.last_mut().unwrap().push('\n');
        value.last_mut().unwrap().push_str(line);

        return key_value_parse_helper(&content[new_line+1..], result);
    }

    let key = &content[..space];
    let value = &content[space+1..new_line];

    insert_or_append(key, value, result);

    return key_value_parse_helper(&content[new_line+1..], result);

}

fn insert_or_append(key: &str, value: &str, result: &mut MultipleValuesMap) {
    let current_value = result.get_mut(key);

    match current_value {
        None => {
            result.insert(key.to_string(), vec![value.to_string()]);
        },
        Some(x) => {
            x.push(value.to_string());
        }
    }
}