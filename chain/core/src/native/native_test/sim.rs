use super::*;
use util::*;
use native::types::*;

#[allow(dead_code, unused_variables)]
pub struct SimpleStorage {
    functions: HashMap<Signature, Box<Function>>,
    uint: U256,
    bytes: Bytes, //Vec<u8>,
    string: String,
    array_u256: Array<U256>,
    map_u256: Map<U256, U256>,
}

#[allow(dead_code)]
impl Contract for SimpleStorage {
    fn get_function(&self, hash: &Signature) -> Option<&Box<Function>> {
        //info!("function hash={:?}", self.functions.get(hash));
        self.functions.get(hash)
    }
}

#[allow(dead_code, unused_variables)]
impl SimpleStorage {
    pub fn new() -> Self {
        let mut contract = SimpleStorage {
            functions: HashMap::<Signature, Box<Function>>::new(),
            uint: U256::from(0),
            bytes: Bytes::new(),  //Vec::new(),
            string: String::new(),
            array_u256: Array::<U256>{ _data: U256::from(0), },
            map_u256: Map::<U256, U256>{
                _key: U256::from(0),
                _value: U256::from(0),
            },
        };
        contract.functions.insert(0, Box::new(SimpleStorage::set));
        contract.functions.insert(1, Box::new(SimpleStorage::get));
        contract.functions.insert(2, Box::new(SimpleStorage::set_bytes));
        contract.functions.insert(3, Box::new(SimpleStorage::get_bytes));
        contract.functions.insert(4, Box::new(SimpleStorage::set_string));
        contract.functions.insert(5, Box::new(SimpleStorage::get_string));
        contract.functions.insert(6, Box::new(SimpleStorage::set_array));
        contract.functions.insert(7, Box::new(SimpleStorage::get_array));
        contract.functions.insert(8, Box::new(SimpleStorage::set_map));
        contract.functions.insert(9, Box::new(SimpleStorage::get_map));
        contract
    }

    pub fn set(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        /*
        if let Some(ref data) = params.data {
            if let Some(data) = data.get(4..36) {
                let _ = ext.set_storage(H256::from(0), H256::from(data));
            }
        }
        */

        if let Some(ref data) = params.data{
            if let Some(data) = data.get(4..36){
                info!("data={:?}, U256::from(data)={:?}", data, U256::from(data));
                let _ = U256::put(ext, &H256::from(0), &U256::from(data));
            }
        }

        Ok(GasLeft::Known(U256::from(0)))
    }

    pub fn get(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
         if let Ok(str) = U256::get(ext, &H256::from(0)){
             info!("get_value={:?}", str);
         }else {
             trace!("exception for func of get_value");
         }
        Ok(GasLeft::Known(U256::from(0)))
    }

    pub fn set_bytes(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        if let Some(ref data) = params.data{
            let n = data.len();
            if let Some(data) = data.get(4..n){
                info!("n={:?}, data={:?}", n, data);
                let _ = Vec::<u8>::put(ext, &H256::from(1), &data.to_vec());
            }
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    pub fn get_bytes(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        if let Ok(bytes) = Vec::<u8>::get(ext, &H256::from(1)){
            info!("get_bytes={:?}", bytes);
        }else {
            trace!("exception for func of get_bytes");
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    pub fn set_string(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        if let Some(ref data) = params.data{
            let n = data.len();
            if let Some(data) = data.get(4..n){
                info!("n={:?}, data={:?}", n, data);
                //let _ = String::put(ext, &H256::from(2), &String::from_utf8(data.to_vec()).unwrap());
                let _ = String::put(ext, &H256::from(2), &data.to_vec().to_hex());
            }
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    pub fn get_string(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        if let Ok(str) = String::get(ext, &H256::from(2)){
            info!("get_string={:?}", *str.as_ref());
        }else {
            trace!("exception for func of get_string");
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    pub fn set_array(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        if let Some(ref data) = params.data {
            //the data of array for solidity
            //the first 4 byte is the hash of function name
            //the second 32 byte is the offset to start of data
            //the third 32 byte is the number of elements of the array
            //the last part is the elements of the array

            //the lowest length of byte of array is (4 + 32 + 32)
            if data.len() >= 68 {
                let count = (data.len() - 4) / 32;
                if let Some(len_array) = data.get(36..68){
                    let len = u64::from(U256::from(len_array));
                    info!("count={:?}, len_array={:?}", count, len);
                    let _ = Array::<U256>::set_len(ext, &H256::from(3), len);
                }

                let mut index = 1;
                while index <= count - 2 {
                    let start:usize = (index-1) * 32 + 68;
                    let end:usize = index * 32 + 68;
                    if let Some(data_array) = data.get(start..end) {
                        info!("index={:?}, U256::data_array={:?}", index as u64, U256::from(data_array));
                        let _ = Array::<U256>::put_item(ext, &H256::from(3), index as u64, &U256::from(data_array));
                    }
                    index = index + 1;
                }
            } else {
                trace!("exception for the length of data");
            }
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    pub fn get_array(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        if let Ok(len_array) = Array::<U256>::get_len(ext, &H256::from(3)){
            info!("len_array={:?}", len_array);
            let mut index:u64 = 1;
            while index <= len_array {
                let data_array = Array::<U256>::get_item(ext, &H256::from(3), index).unwrap();
                info!("index={:?}, data_array={:?}", index, *data_array.as_ref());
                index = index + 1;
            }
        } else {
            trace!("exception for func of get_array");
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    pub fn set_map(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        if let Some(ref data) = params.data {
            let key = data.get(4..36).unwrap();
            let value = data.get(36..68).unwrap();
            info!("key={:?}, value={:?}", U256::from(key), U256::from(value));
            let _ = U256::put(ext, &H256::from(4), &U256::from(key));
            let _ = Map::<U256, U256>::put_item(ext, &H256::from(4), U256::from(key), &U256::from(value));
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    pub fn get_map(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        if let Ok(key) = U256::get(ext, &H256::from(4)){
            info!("get_map of key={:?}", *key.as_ref());
            if let Ok(value) = Map::<U256, U256>::get_item(ext, &H256::from(4), U256::from(*key.as_ref())) {
                info!("get_map of value={:?}", *value.as_ref());
            } else {
                trace!("exception for get_value");
            }
        } else {
            trace!("exception for get_key");
        }

        Ok(GasLeft::Known(U256::from(0)))
    }
}
