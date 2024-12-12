

/// request to Vec<String>
pub fn listify(request:String) -> Vec<String> {
    // splitting into items in list based of pointer to list of target chars
    let lst:Vec<&str> = request.split(&['\n', ':'][..]).collect();
    // conv each item to string for return type
    let lst:Vec<String> = lst.into_iter().map(String::from).collect();

    lst
}