class website:
    def __init__(self, title, aboutText, email, address, contact) -> None:
        self.title = title
        self.aboutText = aboutText
        self.email = email
        self.address = address
        self.contact = contact


class projects: 
    def __init__(self, arr, *args) -> None:
        self.arr = arr

        for arg in args:
            self.arr.append[arg]
        
         
        