import os
import json
import codecs

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
    def __init__(self, title, text, foto) -> None:
        self.title = title
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

print("\n\n")
print("**************** Add Projects to your Website ****************\n")


projectArr = {}


def testForN(bool):
    if (bool != "Y" or bool != "y"):
        bool = input("Press Y to add Projects\n\n")
    elif(bool == "N" or bool == "n"):
        bool = "N"
    elif(bool == "Y" or bool == "y"):
        bool = "Y"
    else:
        testForN(bool)         



bool = "Y"
i = 0
while(bool == "Y" or bool == "y"):
    _project = input("Add a title for your project\n")
    _projectText = input("Add a text for your project\n")
    _fotoUrl = input("Add a photoURL for your project\n")
    listItemProject = project(_project ,_projectText, _fotoUrl)
    projectArr[i] = listItemProject.toJSON()
    i += 1
    bool = input("Press Y to add Projects\n\n")
    testForN(bool)


socialMediaDict = dict()

_boolFacebook = input("Press Y to add facebook to your social media\n")
if(_boolFacebook  != "N" and _boolFacebook != "n"):
    _linkFacebook =  input("Enter the link for your social media\n")
    socialMediaDict['facebook'] = _linkFacebook
    
_boollinkedin = input("Press Y to add linkedin toy our social media\n")
if(_boollinkedin != "N" and _boollinkedin != "n"):
    _linklinkedin =  input("Enter the link for your social media\n")
    socialMediaDict['linkedin'] = _linklinkedin

_boolpinterest = input("Press Y to add pinterest toy our social media\n")
if(_boolpinterest  != "N" and _boolpinterest != "n"):
    _linkpinterest =  input("Enter the link for your social media\n")
    socialMediaDict['pinterest'] = _linkpinterest
    
_booltwitter = input("Press Y to add twitter toy our social media\n")
if(_booltwitter != "N" and _booltwitter != "n"):
    _linktwitter =  input("Enter the link for your social media\n")
    socialMediaDict['twitter'] = _linktwitter
    
boolwhatsapp = input("Press Y to add whatsapp toy our social media\n")
if(boolwhatsapp != "N" and boolwhatsapp != "n"):
    _linkwhatsapp =  input("Enter the link for your social media\n")
    socialMediaDict['whatsapp'] = _linkwhatsapp
    
boolyoutube= input("Press Y to add youtube toy our social media\n")
if(boolyoutube  != "N" and boolyoutube != "n"):
    _linkyoutube =  input("Enter the link for your social media\n")
    socialMediaDict['youtube'] = _linkyoutube
    
boolinstagramm= input("Press Y to add instagramm toy our social media\n")
if(boolinstagramm  != "N" and boolinstagramm != "n"):
    _linkinstagramm =  input("Enter the link for your social media\n")
    socialMediaDict['instagramm'] = _linkinstagramm
    
boolgithub = input("Press Y to add github toy our social media\n")
if(boolgithub != "N" and boolgithub != "n"):
    print
    _linkgithub =  input("Enter the link for your social media\n")
    socialMediaDict['github'] = _linkgithub

jsonObject = [protoWebsite.toJSON(),json.dumps(projectArr),json.dumps(socialMediaDict)
]   
   
with open("data.json", "w") as dataJson:
   dataJson.write(json.dumps(jsonObject)),

print("finished")