import os
import json

class website:
    def __init__(self, title, aboutText, mainColor, secondaryColor, accentColor, backgroundColor) -> None:
        self.title = title
        self.aboutText = aboutText
        self.mainColor = mainColor
        self.secondaryColor = secondaryColor
        self.accentColor = accentColor
        self.backgroundColor = backgroundColor
        
    def toJSON(self):
        return json.dumps(self, default=lambda o: o.__dict__, indent=4)


class project: 
    def __init__(self, text, foto) -> None:
        self.text = text
        self.foto = foto
        
    def toJSON(self):
        return json.dumps(self, default=lambda o: o.__dict__, indent=4)

        # for arg in args:
        #     self.arr.append[arg]
            
# class contact:  
#     def __init__(self, facebook, linkedin, pinterest, twitter, whatsapp, youtube, instagramm, github) -> None:
#         self.facebook = facebook
#         self.linkedin = linkedin
#         self.pinterest = pinterest
#         self.twitter = twitter
#         self.whatsapp = whatsapp
#         self.youtube = youtube
#         self.instagramm = instagramm
#         self.github = github
            

# Greeting
# Display a title bar.
print("**************************************************")
print("***  Application to automate Portfoliowebsites ***")
print("**************************************************")


print("\n\n")

title = input('\nEnter the title\n') 

aboutText = input('\nEnter the about text\n')

mainColor = input('\nEnter the main color\n')

secondaryColor = input('\nEnter the secondary color\n')

accentColor = input('\nEnter the accent color\n')

backgroundColor = input('\nEnter the background color\n')


protoWebsite = website(title, aboutText, mainColor, secondaryColor, accentColor, backgroundColor)

print(protoWebsite.toJSON())



print("\n\n")
print("Add Projects to your Website \n")

bool = "Y"

projectArr = []


def testForN():
    if (bool != "Y" or bool != "y"):
        bool = input("Press Y to add Projects\n\n")
        if(bool != "N" or bool != "n"):
            bool = "N"
        else:
            testForN()         

while(bool == "Y" or bool == "y"):
    _project = input("Add a text for your project\n")
    _fotoUrl = input("Add a photoURL for your project\n")
    listItemProject = project(_project, _fotoUrl)
    
    projectArr.append(listItemProject.toJSON())
    print(listItemProject.toJSON())
    bool = input("Press Y to add Projects\n\n")
    testForN()
    
while(bool == "Y" or bool == "y"):
    socialMediaDict = dict()
    
    _boolFacebook = input("Press Y to add facebook to your social media\n")
    if(_boolFacebook):
        _linkFacebook =  input("Enter the link for your social media\n")
        socialMediaDict['facebook'] = _linkFacebook
        
    _boollinkedin = input("Press Y to add linkedin toy our social media\n")
    if(_boollinkedin):
        _linklinkedin =  input("Enter the link for your social media\n")
        socialMediaDict['linkedin'] = _linklinkedin
   
   
with open("data.json", "w") as dataJson:
    dataJson.write(protoWebsite.toJSON())
    dataJson.write(json.dumps(projectArr, default=lambda o: o.__dict__, indent=4))

input('')