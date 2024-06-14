#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod smart_contracts {
    //use parity_scale_codec::{Encode, Decode};
    //use ink_storage::traits::{PackedLayout, SpreadLayout};
    //use scale_info::TypeInfo;
    use scale_info::prelude::format;
    use scale_info::prelude::string::String;
    use ink_storage::Mapping; 
    /*use ink::storage::{
        traits::ManualKey,
        Mapping,
    };*/
     // Almacenamiento ===========
    // PartialEq y Eq para comparar estructuras, servira para ver si la información ya fue ingresada
    // PackedLayout y SpreadLayout para poder almacenar la información en el contrato de forma eficiente
    // Si está activo el std, se activa TypeInfo. Sirve para la serializacion y deserializacion de datos
    // Es la información que el usuario ingresa
    //#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
    //#[cfg_attr(feature = "std", derive(TypeInfo))]
    /// #[ink::scale_derive(Encode, Decode, TypeInfo)]
    //  #[cfg_attr(
    //     feature = "std",
    //     derive(ink::storage::traits::StorageLayout)
    //  )]

    //#[ink::storage_item]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout,Clone)
    )]
    pub struct UserInfo {
        name: String,
        lastname: String,
        //dni: String, // Servira para asociar la información y rol
        email: String,
    }
   
    // Tipos de roles
    /// Doctor = 1
    /// Paciente = 0
    


    // La información que maneja el smart contract
    #[ink(storage)]
    pub struct AccessControl{
        // La información asociada a la cuenta
        users: Mapping<String, UserInfo>,
        // El rol asociada a la cuenta
        roles: Mapping<String, u8>,
        // Para asociar el paciente cel mapeo de los permisos por doctor
        //permissions: Mapping<String, Mapping<String, bool>>,

    }

    // Almacenamiento ===========
    impl Default for AccessControl {
        fn default() -> Self {
            Self {
                users: Mapping::new(),
                roles: Mapping::new(),
                //permissions: Mapping::new(),
            }
        }
    }
    impl AccessControl {
        // Es el constructor por default, se deben colocar todos los campos de la estructura
        // Constructor
        #[ink(constructor)]
        pub fn default() -> Self {
            Self { 
                users: Mapping::new(),
                roles: Mapping::new(),
                //permissions: Mapping::new(),
            }
        }
        // Constructor
        // Funciones
        // Ver si la información existe, si el dni existe, la información ya fue ingresada
        #[ink(message)]
        pub fn user_exists(&self, dni: String) -> bool {
            self.users.contains(dni)
        }

        // Asignar un rol a un usuario que está creando cuenta
        #[ink(message)]
        pub fn assign_role(&mut self, dni:String, role: u8)->String {
            // Si ya existe el dni al rol que se quiere crear, es true
            let did_user_exist  = if self.roles.get(dni.clone())== Some(role) {true} else {false};
            // El doctor puede crearse una cuenta de paciente
            if !did_user_exist {
                self.roles.insert(dni.clone(), &role);
                return format!("Se asignó el rol {} al DNI {}", role, dni);
            }else{
                return format!("El DNI {} ya tiene el rol {}", dni, role);

            }
        }
        
        // Trae el rol de la cuenta
        #[ink(message)]
        pub fn user_role(&self, dni: String) -> bool {
            self.roles.contains(dni)
        }

        // Añade la información y el usuario
        #[ink(message)]
        pub fn add_user(&mut self, dni: String, user_info: UserInfo) {
            self.users.insert(dni.clone(), &user_info);
        }
    

       /// Functions
        #[ink(message)]
        pub fn give_permission(&mut self, doctor_dni: String) {
            //let patient_dni = self.env().caller().to_string();
            //let is_patient = self.get_role(patient_dni)==Some(1);
            let is_doctor = self.get_role(doctor_dni)==Some(0);
            /*if is_patient {
                if is_doctor {
                    // Si son doctor y paciente, agregar una entrada al mapeo `permissions`
                    let doctor_permissions = self.permissions.entry(patient_dni).or_insert(Mapping::new());
                    doctor_permissions.insert(doctor_dni, true);
                }
            }*/
        }
        
        #[ink(message)]
        pub fn get_role(&mut self, dni: String)->Option<u8> {
            self.roles.get(dni)
        }
    
        
    }

    // Tests
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let access_control = AccessControl::default();
            let dni = String::from("74493233");
            assert_eq!(access_control.user_role(dni), false);
        }

        #[ink::test]
        fn test_add_user_and_user_exists() {
            let mut access = AccessControl::default();
            let dni = String::from("74493233");
            let user_info = UserInfo {
                name: String::from("Alice"),
                lastname: String::from("Prueba"),
                email: String::from("nxhm@gmail.com")
            
            }; // Asume que UserInfo tiene un método new

            // Asegúrate de que el usuario no exista al principio
            assert_eq!(access.user_exists(dni.clone()), false);

            // Añade el usuario
            access.add_user(dni.clone(), user_info);

            // Ahora el usuario debería existir
            assert_eq!(access.user_exists(dni.clone()), true);
        }
    }
}